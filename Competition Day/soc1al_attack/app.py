from flask import Flask
from flask import Flask, flash, redirect, render_template, request, session, abort
import os
 
app = Flask(__name__)
 
@app.route('/')
def home():
    if not session.get('logged_in'):
        return render_template('login.html')
    else:
        return "terrible_pa$$word@apuboh2018"
 
@app.route('/login', methods=['POST'])
def do_admin_login():
    if request.form['Pin'] == '181096':
        session['logged_in'] = True
    else:
        flash('Wrong pin! Please try again')
    return home()
 
if __name__ == "__main__":
    app.secret_key = os.urandom(12)
    app.run(debug=False,host='0.0.0.0', port=4000)