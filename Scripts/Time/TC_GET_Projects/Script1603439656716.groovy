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
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

response = WS.sendRequestAndVerify(findTestObject('Time/GET_Projects'))

WS.verifyResponseStatusCode(response, 200)

WS.verifyElementPropertyValue(response, 'data[3].projectId', '7')

WS.verifyElementPropertyValue(response, 'data[3].projectName', 'febri')

WS.verifyElementPropertyValue(response, 'data[3].customerId', '1')

WS.verifyElementPropertyValue(response, 'data[3].customerName', 'Sasuke Uchiha')

WS.verifyElementPropertyValue(response, 'data[3].description', 'testing')

WS.verifyElementPropertyValue(response, 'data[3].isDeleted', '0')

WS.verifyElementPropertyValue(response, 'data[3].admins.employeeId', '1')

WS.verifyElementPropertyValue(response, 'data[3].admins.name', 'Kijang Satu')

WS.verifyElementPropertyValue(response, 'data[3].activities', null)
