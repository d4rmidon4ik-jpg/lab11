package main

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPingHandler(t *testing.T) {
	r := setupRouter()
	w := httptest.NewRecorder()
	req, _ := http.NewRequest("GET", "/ping", nil)
	r.ServeHTTP(w, req)

	assert.Equal(t, http.StatusOK, w.Code)

	var resp map[string]string
	assert.NoError(t, json.Unmarshal(w.Body.Bytes(), &resp))
	assert.Equal(t, "pong", resp["message"])
	assert.Equal(t, "go", resp["service"])
}

func TestHealthHandler(t *testing.T) {
	r := setupRouter()
	w := httptest.NewRecorder()
	req, _ := http.NewRequest("GET", "/health", nil)
	r.ServeHTTP(w, req)

	assert.Equal(t, http.StatusOK, w.Code)

	var resp map[string]string
	assert.NoError(t, json.Unmarshal(w.Body.Bytes(), &resp))
	assert.Equal(t, "ok", resp["status"])
}