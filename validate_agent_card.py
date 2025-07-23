#!/usr/bin/env python3
"""
Agent Card Validator for OmniAgent A2A Server
Validates the agent card against A2A specification requirements
"""

import json
import sys
import requests
from typing import Dict, Any, List

class AgentCardValidator:
    def __init__(self, base_url: str = "http://localhost:8080"):
        self.base_url = base_url.rstrip('/')
        self.agent_card = None
        
    def fetch_agent_card(self) -> bool:
        """Fetch agent card from server"""
        try:
            response = requests.get(f"{self.base_url}/agent.json", timeout=10)
            response.raise_for_status()
            self.agent_card = response.json()
            return True
        except requests.exceptions.RequestException as e:
            print(f"❌ Failed to fetch agent card: {e}")
            return False
        except json.JSONDecodeError as e:
            print(f"❌ Invalid JSON in agent card: {e}")
            return False
    
    def validate_required_fields(self) -> bool:
        """Validate all required fields are present"""
        if not self.agent_card:
            return False
            
        required_fields = [
            'name', 'description', 'version', 'url', 
            'capabilities', 'skills', 'defaultInputModes', 'defaultOutputModes'
        ]
        
        missing = []
        for field in required_fields:
            if field not in self.agent_card:
                missing.append(field)
        
        if missing:
            print(f"❌ Missing required fields: {missing}")
            return False
            
        print("✅ All required fields present")
        return True
    
    def validate_capabilities(self) -> bool:
        """Validate capabilities structure"""
        if not self.agent_card:
            return False
            
        capabilities = self.agent_card.get('capabilities', {})
        required_capabilities = ['streaming']
        
        for cap in required_capabilities:
            if cap not in capabilities:
                print(f"❌ Missing capability: {cap}")
                return False
        
        print("✅ Capabilities structure valid")
        return True
    
    def validate_skills(self) -> bool:
        """Validate skills array"""
        if not self.agent_card:
            return False
            
        skills = self.agent_card.get('skills', [])
        if not isinstance(skills, list):
            print("❌ Skills should be an array")
            return False
            
        if len(skills) == 0:
            print("⚠️  Skills array is empty")
            
        for i, skill in enumerate(skills):
            if not isinstance(skill, dict):
                print(f"❌ Skill {i} is not an object")
                return False
                
            required_skill_fields = ['id', 'name', 'description', 'tags']
            for field in required_skill_fields:
                if field not in skill:
                    print(f"❌ Skill {i} missing field: {field}")
                    return False
        
        print(f"✅ Skills validation passed ({len(skills)} skills)")
        return True
    
    def validate_url_format(self) -> bool:
        """Validate URL format"""
        if not self.agent_card:
            return False
            
        url = self.agent_card.get('url', '')
        if not url.startswith('http'):
            print(f"❌ Invalid URL format: {url}")
            return False
            
        print(f"✅ URL format valid: {url}")
        return True
    
    def validate_json_schema(self) -> bool:
        """Validate against basic JSON schema"""
        if not self.agent_card:
            return False
            
        try:
            # Basic schema validation
            schema_checks = {
                'name': str,
                'description': str,
                'version': str,
                'url': str,
                'capabilities': dict,
                'skills': list,
                'defaultInputModes': list,
                'defaultOutputModes': list
            }
            
            for field, expected_type in schema_checks.items():
                if field not in self.agent_card:
                    print(f"❌ Missing field: {field}")
                    return False
                    
                actual_value = self.agent_card[field]
                if not isinstance(actual_value, expected_type):
                    print(f"❌ Field {field} should be {expected_type.__name__}, got {type(actual_value).__name__}")
                    return False
            
            print("✅ JSON schema validation passed")
            return True
            
        except Exception as e:
            print(f"❌ Schema validation failed: {e}")
            return False
    
    def display_agent_card_summary(self):
        """Display a summary of the agent card"""
        if not self.agent_card:
            return
            
        print("\n📋 Agent Card Summary:")
        print("=" * 50)
        print(f"Name: {self.agent_card.get('name', 'N/A')}")
        print(f"Description: {self.agent_card.get('description', 'N/A')}")
        print(f"Version: {self.agent_card.get('version', 'N/A')}")
        print(f"URL: {self.agent_card.get('url', 'N/A')}")
        print(f"Skills: {len(self.agent_card.get('skills', []))}")
        print(f"Capabilities: {json.dumps(self.agent_card.get('capabilities', {}), indent=2)}")
        
        skills = self.agent_card.get('skills', [])
        if skills:
            print("\nSkills:")
            for skill in skills[:5]:  # Show first 5 skills
                print(f"  - {skill.get('name', 'N/A')}: {skill.get('description', 'N/A')}")
            if len(skills) > 5:
                print(f"  ... and {len(skills) - 5} more")
    
    def run_validation(self) -> bool:
        """Run all validation tests"""
        print("🔍 Validating Agent Card...")
        print("=" * 50)
        
        if not self.fetch_agent_card():
            return False
        
        tests = [
            self.validate_required_fields,
            self.validate_capabilities,
            self.validate_skills,
            self.validate_url_format,
            self.validate_json_schema
        ]
        
        passed = 0
        total = len(tests)
        
        for test in tests:
            if test():
                passed += 1
        
        self.display_agent_card_summary()
        
        print("\n" + "=" * 50)
        print(f"Validation Results: {passed}/{total} tests passed")
        
        if passed == total:
            print("🎉 All validation tests passed!")
            return True
        else:
            print("❌ Some validation tests failed")
            return False

if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description='Validate OmniAgent A2A Agent Card')
    parser.add_argument('--url', default='http://localhost:8080', 
                       help='Base URL of the A2A server (default: http://localhost:8080)')
    parser.add_argument('--verbose', '-v', action='store_true', 
                       help='Verbose output')
    
    args = parser.parse_args()
    
    validator = AgentCardValidator(args.url)
    
    try:
        success = validator.run_validation()
        sys.exit(0 if success else 1)
    except KeyboardInterrupt:
        print("\n❌ Validation interrupted")
        sys.exit(1)
    except Exception as e:
        print(f"❌ Validation error: {e}")
        sys.exit(1)