echo "Test Case 1: Student-User Release question task: (False, student user cannot release questionnaire)"
curl http://localhost:6789/task/release-question\
 -H "Content-Type:application/json"\
 -d '{"mid":2, "questions":[{"order":0, "q_type":0, "content":"Your Name"}, {"order":1, "q_type":1, "content":"Your Gender", "choices":["Male", "Female"]}, {"order":2, "q_type":2, "content":"Your courses", "choices":["Software-Testing", "VR", "CG", "Data-Mining"]}]}'
echo ""

echo "Test Case 2: Cow-User Release question task: (True)"
curl http://localhost:6789/task/release-question\
 -H "Content-Type:application/json"\
 -d '{"mid":1, "questions":[{"order":0, "q_type":0, "content":"Your Name"}, {"order":1, "q_type":1, "content":"Your Gender", "choices":["Male", "Female"]}, {"order":2, "q_type":2, "content":"Your courses", "choices":["Software-Testing", "VR", "CG", "Data-Mining"]}]}'
echo ""

echo "Test Case 3: Cow-User Release question task Duplication: (False, Cannot rewrite task!)"
curl http://localhost:6789/task/release-question\
 -H "Content-Type:application/json"\
 -d '{"mid":1, "questions":[{"order":0, "q_type":0, "content":"Your Name"}, {"order":1, "q_type":1, "content":"Your Gender", "choices":["Male", "Female"]}, {"order":2, "q_type":2, "content":"Your courses", "choices":["Software-Testing", "VR", "CG", "Data-Mining"]}]}'
echo ""
