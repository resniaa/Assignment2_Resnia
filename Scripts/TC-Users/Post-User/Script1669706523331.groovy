import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import static org.assertj.core.api.Assertions.*
import com.kms.katalon.core.testobject.RequestObject as RequestObject
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import com.kms.katalon.core.webservice.verification.WSResponseManager as WSResponseManager
import groovy.json.JsonSlurper as JsonSlurper

response = WS.sendRequest(findTestObject('Users/POST-User'))
WS.verifyResponseStatusCode(response, 201)

WS.verifyElementPropertyValue(response, 'id', '201')
WS.verifyElementPropertyValue(response, 'name', 'Resnia Trya Muslima')
WS.verifyElementPropertyValue(response, 'username', 'Resnia')
WS.verifyElementPropertyValue(response, 'email', 'resnia@gmail.com')
WS.verifyElementPropertyValue(response, 'address.street', 'Jalan NN')
WS.verifyElementPropertyValue(response, 'address.suite', '16')
WS.verifyElementPropertyValue(response, 'address.city', 'Tangerang Selatan')
WS.verifyElementPropertyValue(response, 'address.zipcode', '15416')
WS.verifyElementPropertyValue(response, 'address.geo.lat', '-37.3159')
WS.verifyElementPropertyValue(response, 'address.geo.lng', '81.1496')
WS.verifyElementPropertyValue(response, 'phone', '085714746591')
WS.verifyElementPropertyValue(response, 'website', 'nianiania.org')
WS.verifyElementPropertyValue(response, 'company.name', 'Nia Company')
WS.verifyElementPropertyValue(response, 'company.catchPhrase', 'Multi-layered client-server neural-net')
WS.verifyElementPropertyValue(response, 'company.bs', 'harness real-time e-markets')