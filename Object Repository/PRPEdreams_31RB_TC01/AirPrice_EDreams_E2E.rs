<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AirPrice_EDreams_E2E</name>
   <tag></tag>
   <elementGuidId>86bb11ac-ecb0-4a4d-b916-b5c83ab2d1eb</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>SOAPAction</name>
      <type>Main</type>
      <value>http://localhost:8080/kestrel/AirService</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic dUFQSTkwNDQxOTkwNzctZjMyYWM2MTM6TmV3cGFzc180VSoyMA==</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soap:Envelope xmlns:soap=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
	&lt;soap:Body>
		&lt;air:AirPriceReq AuthorizedBy=&quot;PANDAY&quot; TargetBranch=&quot;P7113929&quot; FareRuleType=&quot;none&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v50_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot;>
			&lt;com:BillingPointOfSaleInfo OriginApplication=&quot;UAPI&quot;/>
			&lt;air:AirItinerary>
				  &lt;air:AirSegment Key=&quot;9o1YH5sY1DKAwfh+FAAAAA==&quot; Group=&quot;0&quot; ProviderCode=&quot;1G&quot; Carrier=&quot;${Carrier}&quot; FlightNumber=&quot;${FlightNumber}&quot; Origin=&quot;${Origin}&quot; Destination=&quot;${Destination}&quot; DepartureTime=&quot;${DepartureTime}&quot; ArrivalTime=&quot;${ArrivalTime}&quot; Distance=&quot;${Distance}&quot; ETicketability=&quot;Yes&quot; Equipment=&quot;${Equipment}&quot; ChangeOfPlane=&quot;false&quot; ParticipantLevel=&quot;Secure Sell&quot; LinkAvailability=&quot;true&quot; PolledAvailabilityOption=&quot;Cached status used. Polled avail exists&quot; OptionalServicesIndicator=&quot;false&quot; AvailabilitySource=&quot;Q&quot; AvailabilityDisplayType=&quot;Fare Shop/Optimal Shop&quot;>&lt;/air:AirSegment>
               
			&lt;/air:AirItinerary>
			&lt;com:SearchPassenger Code=&quot;ADT&quot; BookingTravelerRef=&quot;1&quot;/>
			 &lt;air:AirPricingCommand>
          &lt;air:AirSegmentPricingModifiers AirSegmentRef=&quot;9o1YH5sY1DKAwfh+FAAAAA==&quot;>
               &lt;air:PermittedBookingCodes>
                  &lt;air:BookingCode Code=&quot;J&quot;/>
               &lt;/air:PermittedBookingCodes>
            &lt;/air:AirSegmentPricingModifiers>
                    &lt;/air:AirPricingCommand>
		&lt;/air:AirPriceReq>
	&lt;/soap:Body>
&lt;/soap:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://WAS-UNIVERSALAPI-33890-100.UT.TVLPORT.COM:33890/kestrel/AirService</soapServiceEndpoint>
   <soapServiceFunction>service</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Carrier</defaultValue>
      <description></description>
      <id>e2eb4353-0a8a-45a2-9f09-46f6aab0f338</id>
      <masked>false</masked>
      <name>Carrier</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.FlightNumber</defaultValue>
      <description></description>
      <id>29e5df34-6f70-4062-b224-9bc656242ab9</id>
      <masked>false</masked>
      <name>FlightNumber</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Origin</defaultValue>
      <description></description>
      <id>1d0ad6c4-eb4e-417b-9d1b-26ff23a71e43</id>
      <masked>false</masked>
      <name>Origin</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Destination</defaultValue>
      <description></description>
      <id>c7dcdf6a-f33f-4bc0-b2b9-b70a20db74ad</id>
      <masked>false</masked>
      <name>Destination</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.DepartureTime</defaultValue>
      <description></description>
      <id>2ad6ebaa-badb-405d-9a09-79a73c963c89</id>
      <masked>false</masked>
      <name>DepartureTime</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ArrivalTime</defaultValue>
      <description></description>
      <id>0431c1aa-a76e-4907-9e16-6be15268ea7b</id>
      <masked>false</masked>
      <name>ArrivalTime</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Distance</defaultValue>
      <description></description>
      <id>8e7baf83-38f8-4cc9-8813-3be2a12c8f31</id>
      <masked>false</masked>
      <name>Distance</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Equipment</defaultValue>
      <description></description>
      <id>53b5a688-5bf0-40f1-9189-31eb950c685b</id>
      <masked>false</masked>
      <name>Equipment</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress>file:/S:/TestData/UAPI_JAVA_Framework/wsdl/air_v51_0/Air.wsdl</wsdlAddress>
</WebServiceRequestEntity>
