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

response = WS.sendRequestAndVerify(findTestObject('Admin/GET_Organization_Information'))

WS.verifyResponseStatusCode(response, 200)

WS.verifyElementPropertyValue(response, 'data.id', '1')

WS.verifyElementPropertyValue(response, 'data.name', 'Ajo Hotel')

WS.verifyElementPropertyValue(response, 'data.taxId', null)

WS.verifyElementPropertyValue(response, 'data.registraionNumber', null)

WS.verifyElementPropertyValue(response, 'data.phone', null)

WS.verifyElementPropertyValue(response, 'data.fax', null)

WS.verifyElementPropertyValue(response, 'data.email', null)

WS.verifyElementPropertyValue(response, 'data.country', 'ID')

WS.verifyElementPropertyValue(response, 'data.province', null)

WS.verifyElementPropertyValue(response, 'data.city', null)

WS.verifyElementPropertyValue(response, 'data.zipCode', null)

WS.verifyElementPropertyValue(response, 'data.street1', null)

WS.verifyElementPropertyValue(response, 'data.street2', null)

WS.verifyElementPropertyValue(response, 'data.note', null)

