#!/usr/bin/env python3
"""
Demo script to showcase LLMTerminal features
"""

import time
import sys
from PyQt5.QtWidgets import QApplication
from main import MainWindow

def demo():
    print("🚀 LLMTerminal Demo")
    print("=" * 30)
    print()
    
    print("✨ New Features Added:")
    print("1. 📋 Tabbed Interface - Main and Settings tabs")
    print("2. 🔐 Settings Tab with API Key Management")
    print("3. 👁️  Show/Hide API key functionality") 
    print("4. 🧪 Test API Keys button")
    print("5. 🗑️  Clear API Keys functionality")
    print("6. ✅ Real-time API key status display")
    print("7. 🔄 Automatic model refresh after saving keys")
    print()
    
    print("🎯 Key Benefits:")
    print("• No need to manually edit .env files")
    print("• Secure password-masked input fields")
    print("• Instant validation of API keys")
    print("• User-friendly error handling")
    print("• Visual status indicators")
    print()
    
    print("📖 How to Use:")
    print("1. Run the application: python main.py")
    print("2. Go to the 'Settings' tab")
    print("3. Enter your OpenAI API key")
    print("4. (Optional) Enter Anthropic API key for Claude models")
    print("5. Click 'Test API Keys' to validate")
    print("6. Click 'Save Settings' to store them")
    print("7. Return to 'Main' tab to use the AI assistant")
    print()
    
    # Optional GUI demo
    response = input("Would you like to see the GUI? (y/n): ")
    if response.lower().startswith('y'):
        print("\n🖥️  Launching GUI demo...")
        app = QApplication(sys.argv)
        window = MainWindow()
        window.show()
        
        print("GUI is now running!")
        print("- Check the 'Settings' tab to configure API keys")
        print("- Use the 'Main' tab for AI interactions")
        print("- Close the window to exit")
        
        sys.exit(app.exec_())
    else:
        print("\n👋 Demo complete! Run 'python main.py' to start the application.")

if __name__ == "__main__":
    demo()
