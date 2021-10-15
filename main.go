package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"net/url"
	"time"
)

//Attachment is the internal structure that contains the information inside a @SlackCommandResponse
type Attachment struct {
	Fallback   string `json:"fallback"`
	Color      string `json:"color"`
	AuthorName string `json:"author_name"`
	ImageURL   string `json:"image_url"`
	Ts         int32  `json:"ts"`
}

//SlackCommandResponse is the json structure that is required to send a slack '/' command response back to the requestor or channel
type SlackCommandResponse struct {
	ResponseType string       `json:"response_type"` //in_channel for response to the full channel
	Attachments  []Attachment `json:"attachments"`
}

//SlackHandler handles Command Posts of Slack
func SlackHandler(res http.ResponseWriter, req *http.Request) {
	req.ParseForm()
	token := req.FormValue("token")
	//TODO check if the token equals the token of the registered application.
	if token != "nCVIdDMdaDuRk2FulVrdA7YY" {
		http.Error(res, "You are not allowed to use this service", http.StatusUnauthorized)
	}

	tegeltext := req.FormValue("text")
	username := req.FormValue("user_name")
	timestamp := int32(time.Now().Unix())
	imageURL := fmt.Sprintf("http://tegelspreukmaker.nl/tegelgenerator.php?tt=%s&fs=100&yp=50&lh=120&ltt=13&tg=1&tk=003370&bf=&fx=0&fy=0&se=0", url.QueryEscape(tegeltext))
	slackCommandResponse := SlackCommandResponse{"in_channel", []Attachment{Attachment{tegeltext, "#36a64f", username, imageURL, timestamp}}}
	res.Header().Set("Content-Type", "application/json; charset=utf-8")
	json.NewEncoder(res).Encode(slackCommandResponse)
}

func main() {
	log.Println("Starting HTTP server on 5000")
	http.HandleFunc("/slack", SlackHandler)
	err := http.ListenAndServe(":5000", nil)
	if err != nil {
		log.Fatal("ListenAndServe: ", err)
	}
}
