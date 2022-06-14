<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Air LFS</name>
   <tag></tag>
   <elementGuidId>bf420c1e-ef50-4531-a6e2-27e6ff5224c0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic dUFQSTc2Njc4NjIyMTAtNzAzZWE4ODM6VHk0L19CeDNRNg==</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
	&lt;soapenv:Body>
		&lt;air:LowFareSearchReq AuthorizedBy=&quot;SUVO&quot; TargetBranch=&quot;${TargetBranch}&quot; SolutionResult=&quot;true&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v50_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot;>
			&lt;com:BillingPointOfSaleInfo OriginApplication=&quot;UAPI&quot;/>
			&lt;air:SearchAirLeg>
				&lt;air:SearchOrigin>
					&lt;com:Airport Code=&quot;${Origin}&quot;/>
				&lt;/air:SearchOrigin>
				&lt;air:SearchDestination>
					&lt;com:Airport Code=&quot;${Destination}&quot;/>
				&lt;/air:SearchDestination>
				&lt;air:SearchDepTime PreferredTime=&quot;${DepartureDate}&quot;/>
			&lt;/air:SearchAirLeg>
			&lt;air:AirSearchModifiers>
				&lt;air:PreferredProviders>
					&lt;com:Provider Code=&quot;${Provider}&quot;/>
				&lt;/air:PreferredProviders>
				&lt;air:PermittedCarriers>
					&lt;com:Carrier Code=&quot;${Carrier}&quot;/>
				&lt;/air:PermittedCarriers>
			&lt;/air:AirSearchModifiers>
			&lt;com:SearchPassenger Code=&quot;ADT&quot;/>
		&lt;/air:LowFareSearchReq>
	&lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>${GlobalVariable.uAPIEndpoint}</soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.TargetBranch</defaultValue>
      <description></description>
      <id>f9c7205c-e6e1-40d3-9501-460c8482d463</id>
      <masked>false</masked>
      <name>TargetBranch</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Provider</defaultValue>
      <description></description>
      <id>a5f242d1-3ff4-42dd-a548-cc8646b48550</id>
      <masked>false</masked>
      <name>Provider</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Origin</defaultValue>
      <description></description>
      <id>44285b10-21c0-495b-a81f-c796d7b4bc3a</id>
      <masked>false</masked>
      <name>Origin</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Destination</defaultValue>
      <description></description>
      <id>75e02724-0a24-4930-b1e2-28cd3a69d173</id>
      <masked>false</masked>
      <name>Destination</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Carrier</defaultValue>
      <description></description>
      <id>b7c38fee-f80f-4aef-bde8-285919e64e27</id>
      <masked>false</masked>
      <name>Carrier</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.DepartureDate</defaultValue>
      <description></description>
      <id>dcf83229-0705-4425-bcb8-be494ad1859d</id>
      <masked>false</masked>
      <name>DepartureDate</name>
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

println(&quot;FlightDetailsList : &quot; + (WS.getElementText(response, 'Body.LowFareSearchRsp.FlightDetailsList.FlightDetails')))

println(&quot;SegmentList : &quot; + (WS.getElementText(response, 'Body.LowFareSearchRsp.AirSegmentList.AirSegment')))

println(&quot;FareInfoList : &quot; + (WS.getElementText(response, 'Body.LowFareSearchRsp.FareInfoList.FareInfo')))</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
