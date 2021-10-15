package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"net/url"
	"strings"
	"testing"
)

func TestHandleSlackReturnsWithGeneratedImageURL(t *testing.T) {

	form := url.Values{}
	form.Add("token", "nCVIdDMdaDuRk2FulVrdA7YY")
	form.Add("team_id", "T0001")
	form.Add("team_domain", "example")
	form.Add("enterprise_id", "E0001")
	form.Add("enterprise_name", "Globular%20Construct%20Inc")
	form.Add("channel_id", "2147483705")
	form.Add("channel_name", "test")
	form.Add("user_id", "U2147483697")
	form.Add("user_name", "Steve")
	form.Add("command", "/tegeltje")
	form.Add("text", "beter een vogel in de hand dan 10 in de lucht")
	form.Add("response_url", "https://hooks.slack.com/commands/1234/5678")

	requestData := strings.NewReader(form.Encode())
	request, _ := http.NewRequest("POST", "/slack", requestData)
	request.Header.Set("Content-Type", "application/x-www-form-urlencoded")

	response := httptest.NewRecorder()

	SlackHandler(response, request)

	if response.Code != http.StatusOK {
		t.Fatalf("Non-expected status code%v:\n\tbody: %v", "200", response.Code)
	}

	if response.Header().Get("Content-Type") != "application/json; charset=utf-8" {
		t.Errorf("Invalid content type %s", response.Header().Get("Content-Type"))
	}
	var f interface{}
	json.NewDecoder(response.Body).Decode(&f)
	m := f.(map[string]interface{})

	if m["response_type"] != "in_channel" {
		t.Errorf("response_type is not in_channel but %s", m["response_type"])
	}

	attachments := m["attachments"]
	fmt.Println(attachments)
	// if attachments[0]["color"] != "#36a64f" {
	// 	t.Errorf("color is not #36a64f but %s", m["color"])
	// }
	// responseBody := `{"response_type":"in_channel","attachments":[{"fallback":"beter een vogel in de hand dan 10 in de lucht","color":"#36a64f","author_name":"Steve","image_url":"http://tegelspreukmaker.nl/tegelgenerator.php?tt=beter+een+vogel+in+de+hand+dan+10+in+de+lucht\u0026fs=100\u0026yp=50\u0026lh=120\u0026ltt=13\u0026tg=1\u0026tk=003370\u0026bf=\u0026fx=0\u0026fy=0\u0026se=0","ts":1497904587}]}`
	// if response.Body.String() != responseBody {
	// 	t.Errorf("%s does not equal \n%s", response.Body.String(), responseBody)
	// }
}
