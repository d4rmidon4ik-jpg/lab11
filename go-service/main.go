package main

import (
	"log"
	"net/http"
	"os"

	"github.com/gin-gonic/gin"
)

func setupRouter() *gin.Engine {
	r := gin.New()
	r.Use(gin.Recovery())
	r.Use(gin.Logger())

	r.GET("/ping", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"message": "pong",
			"service": "go",
		})
	})

	r.GET("/health", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{"status": "ok"})
	})

	return r
}

func main() {
	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}

	router := setupRouter()
	log.Printf("Go service starting on :%s", port)
	if err := router.Run(":" + port); err != nil {
		log.Fatalf("failed to start server: %v", err)
	}
}