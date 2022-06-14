<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GBC1</name>
   <tag></tag>
   <elementGuidId>dd6b454d-5d78-4d1c-b207-b1826ab00238</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;SOAP-ENV:Envelope xmlns:SOAP-ENV=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
	&lt;SOAP-ENV:Header>
		&lt;t:Transaction xmlns:t=&quot;xxs&quot;>
			&lt;tc>
	&lt;iden u=&quot;usagateway&quot; p=&quot;usagateway01us&quot;/>
	&lt;provider session=&quot;W1XMLDEV&quot; pcc=&quot;KJJ&quot;>Worldspan&lt;/provider>
	&lt;trace/>
&lt;/tc>
		&lt;/t:Transaction>
	&lt;/SOAP-ENV:Header>
	&lt;SOAP-ENV:Body>
		&lt;ns1:ProviderTransaction xmlns:ns1=&quot;xxs&quot;>
			&lt;REQ>
				&lt;GBC1>
					&lt;MSG_VERSION>1&lt;/MSG_VERSION>
					&lt;VARIABLE_COUNT>0001&lt;/VARIABLE_COUNT>
					&lt;GROUP_INFO>
						&lt;GROUP_COUNT>05&lt;/GROUP_COUNT>
						&lt;GROUP_NAME>MYGROUP&lt;/GROUP_NAME>
					&lt;/GROUP_INFO>
					&lt;PASSENGER_DATA>
						&lt;NAME_POSITION>2&lt;/NAME_POSITION>
						&lt;LAST_FIRST_MIDDLE>PAX/ONE&lt;/LAST_FIRST_MIDDLE>
					&lt;/PASSENGER_DATA>
					&lt;PASSENGER_DATA>
						&lt;NAME_POSITION>3&lt;/NAME_POSITION>
						&lt;LAST_FIRST_MIDDLE>PAX/TWO&lt;/LAST_FIRST_MIDDLE>
					&lt;/PASSENGER_DATA>
					&lt;PASSENGER_DATA>
						&lt;NAME_POSITION>4&lt;/NAME_POSITION>
						&lt;LAST_FIRST_MIDDLE>PAX/THREE&lt;/LAST_FIRST_MIDDLE>
					&lt;/PASSENGER_DATA>
					&lt;PASSENGER_DATA>
						&lt;NAME_POSITION>5&lt;/NAME_POSITION>
						&lt;LAST_FIRST_MIDDLE>PAX/FOUR&lt;/LAST_FIRST_MIDDLE>
					&lt;/PASSENGER_DATA>
					&lt;PNR_DATA>
						&lt;PHONE_FIELDS>
							&lt;PHONE_PSEUDO_CITY>6YE&lt;/PHONE_PSEUDO_CITY>
							&lt;PHONE_FIELD>8168910000&lt;/PHONE_FIELD>
						&lt;/PHONE_FIELDS>
						&lt;OWNER_PARTITION>1P&lt;/OWNER_PARTITION>
						&lt;RECEIVED_FROM>TS&lt;/RECEIVED_FROM>
						&lt;FOP>CA&lt;/FOP>
						&lt;REMARK>TEST MSG&lt;/REMARK>
					&lt;/PNR_DATA>
					&lt;END_OPTION>E&lt;/END_OPTION>
					&lt;TICKET_OPTION>7T/&lt;/TICKET_OPTION>
					&lt;LEG_COUNT>1&lt;/LEG_COUNT>
					&lt;SEGMENT_INFO>
						&lt;LEG_NUM>1&lt;/LEG_NUM>
						&lt;SEGMENT_INDICATOR>N&lt;/SEGMENT_INDICATOR>
						&lt;AVAIL_LOCATION>N&lt;/AVAIL_LOCATION>
						&lt;AVAIL_INFO>
							&lt;AVAIL_DEP_AIRPORT>FRA&lt;/AVAIL_DEP_AIRPORT>
							&lt;AVAIL_AIRLINE>KL&lt;/AVAIL_AIRLINE>
							&lt;AVAIL_DEP_CLASS>Y&lt;/AVAIL_DEP_CLASS>
							&lt;AVAIL_ARRIVAL_AIRPORT>AMS&lt;/AVAIL_ARRIVAL_AIRPORT>
							&lt;AVAIL_DEP_DAY>20&lt;/AVAIL_DEP_DAY>
							&lt;AVAIL_DEP_MONTH>FEB&lt;/AVAIL_DEP_MONTH>
							&lt;AVAIL_DEP_TIME>0800&lt;/AVAIL_DEP_TIME>
						&lt;/AVAIL_INFO>
						&lt;AIRLINE_CODE>KL&lt;/AIRLINE_CODE>
						&lt;FLIGHT_NUM>1&lt;/FLIGHT_NUM>
						&lt;DEP_CLASS>Y&lt;/DEP_CLASS>
						&lt;DEP_DATE>
							&lt;DEP_DAY>20&lt;/DEP_DAY>
							&lt;DEP_MONTH>FEB&lt;/DEP_MONTH>
						&lt;/DEP_DATE>\t &lt;DEP_TIME>
							&lt;DEP_HOUR>11&lt;/DEP_HOUR>
							&lt;DEP_MIN>05&lt;/DEP_MIN>
						&lt;/DEP_TIME>
						&lt;DEP_AIRPORT>AMS&lt;/DEP_AIRPORT>
						&lt;ARRIVAL_DATE>
							&lt;ARRIV_DAY>21&lt;/ARRIV_DAY>
							&lt;ARRIV_MONTH>FEB&lt;/ARRIV_MONTH>
						&lt;/ARRIVAL_DATE>
						&lt;ARRIVAL_TIME>
							&lt;ARRIV_HOUR>16&lt;/ARRIV_HOUR>
							&lt;ARRIV_MIN>15&lt;/ARRIV_MIN>
						&lt;/ARRIVAL_TIME>
						&lt;ARRIV_AIRPORT>FRA&lt;/ARRIV_AIRPORT>
					&lt;/SEGMENT_INFO>
				&lt;/GBC1>
			&lt;/REQ>
		&lt;/ns1:ProviderTransaction>
	&lt;/SOAP-ENV:Body>
&lt;/SOAP-ENV:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>${GlobalVariable.Endpoint}</soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Session</defaultValue>
      <description></description>
      <id>a5cfd42a-6953-4424-b734-d8f851e56190</id>
      <masked>false</masked>
      <name>Session</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.PCC</defaultValue>
      <description></description>
      <id>ac6baefa-85e0-482f-b2c7-2f07b49356f7</id>
      <masked>false</masked>
      <name>PCC</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Password</defaultValue>
      <description></description>
      <id>414e9fe0-4feb-4ab4-b80b-02d2cfbdab2b</id>
      <masked>false</masked>
      <name>Password</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Username</defaultValue>
      <description></description>
      <id>7e952581-3f2f-4821-b03a-cb84edbd4936</id>
      <masked>false</masked>
      <name>Username</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Air_1G').getValue(2, 1)</defaultValue>
      <description></description>
      <id>156d3e38-cf05-4e77-820c-bc9ed7620ee3</id>
      <masked>false</masked>
      <name>Origin</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Air_1G').getValue(3, 1)</defaultValue>
      <description></description>
      <id>bca40c1c-f925-433a-893c-fbdb3f17e4dc</id>
      <masked>false</masked>
      <name>Destination</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Carrier</defaultValue>
      <description></description>
      <id>f8238d97-b6a4-4123-b1d6-b8f525e71667</id>
      <masked>false</masked>
      <name>Carrier</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Provider</defaultValue>
      <description></description>
      <id>bc1f1fe2-7b12-4977-8777-b3b24197ad26</id>
      <masked>false</masked>
      <name>Provider</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
