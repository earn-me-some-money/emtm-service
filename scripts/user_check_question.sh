echo "Test Case 1: (True)"
curl http://localhost:6789/task/question-naire\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"task_mid":1, "userid":"wechat12306", "student_id":2}'
echo ""


