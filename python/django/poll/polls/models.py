from datetime import timedelta
from django.utils import timezone
from django.db import models


class Question(models.Model):
    text = models.CharField(max_length=200)
    pub_date = models.DateTimeField("date_published")

    def __str__(self):
        return self.text

    def was_published_recently(self):
        return (self.pub_date >= timezone.now() - timedelta(days=1)) and (
            self.pub_date < timezone.now()
        )

    was_published_recently.admin_order_field = "pub_date"
    was_published_recently.boolean = True
    was_published_recently.short_description = "Published recently?"


class Choice(models.Model):
    question = models.ForeignKey(Question, on_delete=models.CASCADE)
    text = models.CharField(max_length=200)
    votes = models.IntegerField(default=0)

    def __str__(self):
        return self.text
