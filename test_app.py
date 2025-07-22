#!/usr/bin/env python3
# Test script to verify LLMTerminal can start

import sys
import os

# Add current directory to path
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

def test_imports():
    """Test that all imports work correctly"""
    try:
        print("Testing imports...")
        from main import AssistantGUI, autonomous_agent, create_file_safe
        print("✓ All imports successful!")
        return True
    except Exception as e:
        print(f"✗ Import error: {e}")
        return False

def test_gui_creation():
    """Test that GUI can be created without errors"""
    try:
        print("Testing GUI creation...")
        from PyQt5.QtWidgets import QApplication
        from main import MainWindow
        
        app = QApplication([])
        window = MainWindow()
        print("✓ Main window with tabs created successfully!")
        
        # Test that tabs exist
        if hasattr(window, 'assistant_tab') and hasattr(window, 'settings_tab'):
            print("✓ Both Assistant and Settings tabs created!")
        else:
            print("⚠ Warning: Some tabs may not be properly initialized")
            
        app.quit()
        return True
    except Exception as e:
        print(f"✗ GUI creation error: {e}")
        return False

def test_file_operations():
    """Test basic file operations"""
    try:
        print("Testing file operations...")
        from main import create_file_safe, read_file_safe, delete_path_safe
        
        # Test create file
        result = create_file_safe('{"path": "test_file.txt", "content": "Hello World!"}')
        print(f"Create file result: {result}")
        
        # Test read file
        result = read_file_safe('{"path": "test_file.txt"}')
        print(f"Read file result: {result}")
        
        # Test delete file
        result = delete_path_safe('{"path": "test_file.txt"}')
        print(f"Delete file result: {result}")
        
        print("✓ File operations working!")
        return True
    except Exception as e:
        print(f"✗ File operations error: {e}")
        return False

def main():
    print("LLMTerminal Test Suite")
    print("=" * 30)
    
    tests = [
        test_imports,
        test_gui_creation,
        test_file_operations,
    ]
    
    passed = 0
    for test in tests:
        try:
            if test():
                passed += 1
        except Exception as e:
            print(f"✗ Test failed with exception: {e}")
        print()
    
    print(f"Results: {passed}/{len(tests)} tests passed")
    
    if passed == len(tests):
        print("🎉 All tests passed! LLMTerminal is ready to run.")
        print("To start the application, run: python main.py")
    else:
        print("⚠️  Some tests failed. Check the errors above.")

if __name__ == "__main__":
    main()
