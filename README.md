# Artificial App

```bash
curl "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key=GEMINI_API_KEY" \
-H 'Content-Type: application/json' \
-X POST \
-d '{
  "contents": [{
    "parts":[{"text": "Explain how AI works"}]
    }]
   }'

"system_instruction": { "parts": { "text": "" } }

"contents": [{
           "parts":[
               {"text": "Write a story about a magic backpack."}
           ]
       }],
       "safetySettings": [
           {
               "category": "HARM_CATEGORY_DANGEROUS_CONTENT",
               "threshold": "BLOCK_ONLY_HIGH"
           }
       ],
       "generationConfig": {
           "stopSequences": [
               "Title"
           ],
           "temperature": 1.0,
           "maxOutputTokens": 800,
           "topP": 0.8,
           "topK": 10
       }

      "contents": [
       {"role":"user",
        "parts":[{
          "text": "Hello"}]},
       {"role": "model",
        "parts":[{
          "text": "Great to meet you. What would you like to know?"}]},
       {"role":"user",
        "parts":[{
          "text": "I have two dogs in my house. How many paws are in my house?"}]},
     ]
```

Response

````json
{
  "candidates": [
    {
      "content": {
        "parts": [
          {
            "text": "```html\n<!DOCTYPE html>\n<html>\n<head>\n  <title>Todo App</title>\n</head>\n<body>\n  <h1>Todo List</h1>\n  <input type=\"text\" id=\"new-todo\" placeholder=\"Add new todo\">\n  <button id=\"add-todo-button\">Add</button>\n  <ul id=\"todo-list\">\n  </ul>\n\n  <script>\n    document.getElementById('add-todo-button').addEventListener('click', function() {\n      fetch('/interaction', {\n        method: 'POST',\n        headers: {\n          'Content-Type': 'application/json',\n        },\n        body: JSON.stringify({ element_id: 'add-todo-button' }),\n      });\n    });\n\n    document.getElementById('new-todo').addEventListener('input', function() {\n      fetch('/interaction', {\n        method: 'POST',\n        headers: {\n          'Content-Type': 'application/json',\n        },\n        body: JSON.stringify({ element_id: 'new-todo' }),\n      });\n    });\n\n  </script>\n</body>\n</html>\n```"
          }
        ],
        "role": "model"
      },
      "finishReason": "STOP",
      "safetyRatings": [
        {
          "category": "HARM_CATEGORY_HATE_SPEECH",
          "probability": "NEGLIGIBLE"
        },
        {
          "category": "HARM_CATEGORY_DANGEROUS_CONTENT",
          "probability": "NEGLIGIBLE"
        },
        {
          "category": "HARM_CATEGORY_HARASSMENT",
          "probability": "NEGLIGIBLE"
        },
        {
          "category": "HARM_CATEGORY_SEXUALLY_EXPLICIT",
          "probability": "NEGLIGIBLE"
        }
      ],
      "avgLogprobs": -0.026208528966614693
    }
  ],
  "usageMetadata": {
    "promptTokenCount": 132,
    "candidatesTokenCount": 264,
    "totalTokenCount": 396,
    "promptTokensDetails": [
      {
        "modality": "TEXT",
        "tokenCount": 132
      }
    ],
    "candidatesTokensDetails": [
      {
        "modality": "TEXT",
        "tokenCount": 264
      }
    ]
  },
  "modelVersion": "gemini-2.0-flash"
}
````
