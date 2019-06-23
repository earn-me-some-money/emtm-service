echo "Test-Case 1: Cow User Verify:"
curl http://localhost:6789/user/verify\
 -H "Content-Type:application/json"\
 -d '{"image_data":"", "verify_mode":false, "user_id":"16340023", "organization":"中山大学"}'
echo ""
