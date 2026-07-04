commit="$1"

git remote set-url origin https://learn.zone01oujda.ma/git/iabid/piscine-rust.git
git config --global user.name "iabid"
git config --global user.email "ilyassabid11@gmail.com"
git config --global credential.helper store
git add .
git commit -m "$commit"
git push --set-upstream origin main
git remote add backup 'https://github.com/ilyas-11/piscine-rust'
git push backup